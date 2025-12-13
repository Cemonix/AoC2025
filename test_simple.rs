// Simple test case
fn main() {
    // Shape 4 from input:
    // ###
    // #..
    // ###

    // This should be cells: (0,0), (0,1), (0,2), (1,0), (2,0), (2,1), (2,2)
    let cells = vec![(0,0), (0,1), (0,2), (1,0), (2,0), (2,1), (2,2)];

    println!("Shape 4 cells: {:?}", cells);
    println!("Cell count: {}", cells.len()); // Should be 7

    // Test: can we fit 2 of these in a 4x4 grid?
    // Grid has 16 cells, 2 shapes have 14 cells
    println!("Grid size: 4x4 = 16 cells");
    println!("2 shapes need: 2 * 7 = 14 cells");
    println!("Leftover: 2 cells");

    // According to the problem, this should fit. Let's check the example:
    // AAA.
    // ABAB
    // ABAB
    // .BBB

    println!("\nExpected layout:");
    println!("AAA.");
    println!("ABAB");
    println!("ABAB");
    println!(".BBB");
}
