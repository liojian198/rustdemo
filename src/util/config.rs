pub(crate) fn add_four(a: i32) -> i32 {
    a+4
}

pub mod my_module {
    pub(in crate::util::config::my_module) fn module_function () {

    }

    pub fn public_function () {
        crate::util::config::add_four(1);
        module_function();
    }
}

mod another_module {
    use crate::util::config::add_four;
    pub fn another_function() {
        add_four(2);
        //super::my_module::module_function();
    }

}