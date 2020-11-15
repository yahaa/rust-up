mod types;
mod types_grpc;

use psutil::*;

use std::collections::LinkedList;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use types_grpc::*;

struct NodeMetricsImpl {
    metrics: Arc<Mutex<Metrics>>,
}

struct Metrics {
    cpu_cores: i32,
    mem_total: u64,
    max_points: usize,
    cpu: LinkedList<types::StatusPoint>,
    mem: LinkedList<types::StatusPoint>,
    load: types::LoadInfo,
}

impl NodeMetricsImpl {
    pub fn new() -> NodeMetricsImpl {
        let metrics = Metrics {
            max_points: 120,
            cpu_cores: 0,
            mem_total: 0,
            cpu: LinkedList::new(),
            mem: LinkedList::new(),
            load: types::LoadInfo::new(),
        };

        let metrics = Arc::new(Mutex::new(metrics));
        let metrics_impl = NodeMetricsImpl {
            metrics: Arc::clone(&metrics),
        };

        thread::spawn(move || {
            NodeMetricsImpl::collect(metrics);
        });

        metrics_impl
    }

    pub fn collect(metrics: Arc<Mutex<Metrics>>) -> ! {
        loop {
            let mut cpu_p_collector = cpu::CpuPercentCollector::new().unwrap();

            thread::sleep(time::Duration::from_secs(60));

            let cpu_p = cpu_p_collector.cpu_percent().unwrap();
            let cores = cpu::cpu_count();

            let mem = memory::virtual_memory().unwrap();

            let load_info = host::loadavg().unwrap();

            let mut metrics = metrics.lock().unwrap();

            metrics.cpu_cores = cores as i32;
            metrics.mem_total = mem.total();
            metrics.load.cores = cores as i32;
            metrics.load.load1 = load_info.one;
            metrics.load.load5 = load_info.five;
            metrics.load.load1 = load_info.fifteen;

            let mut cpu_point = types::StatusPoint::new();
            cpu_point.set_percents(cpu_p as f64);

            metrics.cpu.push_front(cpu_point);

            if metrics.cpu.len() > metrics.max_points {
                metrics.cpu.pop_back();
            }

            let mut mem_point = types::StatusPoint::new();
            mem_point.set_percents(mem.percent() as f64);
            mem_point.set_usage(mem.used());

            metrics.mem.push_front(mem_point);

            if metrics.mem.len() > metrics.max_points {
                metrics.mem.pop_back();
            }
        }
    }
}

impl NodeMetrics for NodeMetricsImpl {
    fn summary(
        &self,
        _: ::grpc::ServerHandlerContext,
        _: ::grpc::ServerRequestSingle<types::QueryNodeInfo>,
        resp: ::grpc::ServerResponseUnarySink<types::NodeInfo>,
    ) -> ::grpc::Result<()> {
        let metrics = self.metrics.lock().unwrap();

        let mut n = types::NodeInfo::new();

        let mut load_info = types::LoadInfo::new();
        load_info.set_cores(metrics.load.cores);

        let mut cpu_points = protobuf::RepeatedField::new();
        for p in metrics.cpu.iter() {
            let mut po = types::StatusPoint::new();
            po.set_usage(p.usage);
            po.set_percents(p.percents);

            cpu_points.push(po);
        }

        let mut cpu_info = types::CPUInfo::new();
        cpu_info.cores = metrics.cpu_cores;
        cpu_info.set_status(cpu_points);

        let mut mem_points = protobuf::RepeatedField::new();
        for p in metrics.mem.iter() {
            let mut po = types::StatusPoint::new();
            po.set_usage(p.usage);
            po.set_percents(p.percents);

            mem_points.push(po);
        }
        let mut mem_info = types::MemoryInfo::new();
        mem_info.total = metrics.mem_total;
        mem_info.set_status(mem_points);

        n.set_load(load_info);
        n.set_cpu(cpu_info);
        n.set_memory(mem_info);

        resp.finish(n)
    }

    fn memory(
        &self,
        _: ::grpc::ServerHandlerContext,
        _: ::grpc::ServerRequestSingle<types::QueryMemoryInfo>,
        resp: ::grpc::ServerResponseUnarySink<types::MemoryInfo>,
    ) -> ::grpc::Result<()> {
        let metrics = self.metrics.lock().unwrap();

        let mut mem_points = protobuf::RepeatedField::new();
        for p in metrics.mem.iter() {
            let mut po = types::StatusPoint::new();
            po.set_usage(p.usage);
            po.set_percents(p.percents);

            mem_points.push(po);
        }
        let mut mem_info = types::MemoryInfo::new();
        mem_info.total = metrics.mem_total;
        mem_info.set_status(mem_points);

        resp.finish(mem_info)
    }

    fn cpu(
        &self,
        _: ::grpc::ServerHandlerContext,
        _: ::grpc::ServerRequestSingle<types::QueryCPUInfo>,
        resp: ::grpc::ServerResponseUnarySink<types::CPUInfo>,
    ) -> ::grpc::Result<()> {
        let metrics = self.metrics.lock().unwrap();

        let mut cpu_points = protobuf::RepeatedField::new();
        for p in metrics.cpu.iter() {
            let mut po = types::StatusPoint::new();
            po.set_usage(p.usage);
            po.set_percents(p.percents);

            cpu_points.push(po);
        }
        let mut cpu_info = types::CPUInfo::new();
        cpu_info.cores = metrics.cpu_cores;
        cpu_info.set_status(cpu_points);

        resp.finish(cpu_info)
    }

    fn load(
        &self,
        _: ::grpc::ServerHandlerContext,
        _: ::grpc::ServerRequestSingle<types::QueryLoadInfo>,
        resp: ::grpc::ServerResponseUnarySink<types::LoadInfo>,
    ) -> ::grpc::Result<()> {
        let metrics = self.metrics.lock().unwrap();

        let mut load_info = types::LoadInfo::new();
        load_info.cores = metrics.load.cores;
        load_info.set_load1(metrics.load.load1);
        load_info.set_load5(metrics.load.load5);
        load_info.set_load15(metrics.load.load15);

        resp.finish(load_info)
    }
}

fn main() {
    let metric_impl = NodeMetricsImpl::new();

    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(5000);
    server.add_service(types_grpc::NodeMetricsServer::new_service_def(metric_impl));

    let _server = server.build().expect("server");

    println!("server started on port {}", 5000);

    loop {
        thread::park();
    }
}
