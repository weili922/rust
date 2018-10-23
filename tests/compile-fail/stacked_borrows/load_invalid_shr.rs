// Make sure that we cannot load from memory a `&` that got already invalidated.
fn main() {
    let x = &mut 42;
    let xraw = x as *mut _;
    let xref = unsafe { &*xraw };
    let xref_in_mem = Box::new(xref);
    let _val = *x; // invalidate xraw
    let _val = *xref_in_mem; //~ ERROR Shr reference with non-reactivatable tag: Location should be frozen
}
