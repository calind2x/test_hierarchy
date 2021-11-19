use ws1::module_1::business_logic_outer::test_outer;
use ws1::module_1::module_1_1::business_logic_inner::test_inner;

fn main() {
    println!("THIS IS MAIN!");
    test_outer();
    test_inner();
}
