mod korean;
mod english;

use module_import::japanese::*;
// use module_import::japanese::hello_ja; 이렇게 해도됌.

fn main() {
    korean::hello_ko();
    english::hello_en();
    hello_ja();
}
