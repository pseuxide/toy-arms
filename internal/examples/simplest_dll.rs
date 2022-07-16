/*
This is the demonstration of how to make the simplest hack with toy-arms.
*/
// A neat macro which defines entry point instead of you.
// Also, you dont have to alloc/free console by yourself, console will show up when u compile into debug build.

internal::create_entrypoint!(hack_main_thread);

// Main thread
fn hack_main_thread() {
    // YOUR STUNNING CODE'S SUPPOSED TO BE HERE;
    for i in 0..30000 {
        println!("using toy-arms {}", i);
    }
}
