mod A;
mod B;
// fn main() {
//     A::a();
//     B::b();
//     B::C::c();
// }

use moduse::outer;

fn main() {
    outer::a();
    outer::b();
    outer::inner::c();
    outer::inner::d();
}
