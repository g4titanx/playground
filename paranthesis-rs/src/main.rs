// A simple Rust implementation of a stack-based language that reads a string once and uses a single stack for memory.
// This language can check if parentheses are balanced but cannot reliably check both parentheses
// and brackets, demonstrating it is not Turing complete.

use std::vec::Vec;

// Checks if every '(' has a matching ')' using a single stack.
// Returns true if balanced, false otherwise.
// Example: "((()))" → true, "(()" → false, ")((" → false
//
// `check_parentheses("((()))")`:
// Initial: stack = []
//   [ ]
// Step 1: Read '(' → stack.push('(')
//   [ ( ] ← Push arrow
// Step 2: Read '(' → stack.push('(')
//   [ ( , ( ] ← Push arrow
// Step 3: Read '(' → stack.push('(')
//   [ ( , ( , ( ] ← Push arrow
// Step 4: Read ')' → stack.pop() (get '(', matches)
//   [ ( , ( ] ← Pop arrow
// Step 5: Read ')' → stack.pop() (get '(', matches)
//   [ ( ] ← Pop arrow
// Step 6: Read ')' → stack.pop() (get '(', matches)
//   [ ] ← Pop arrow
fn check_parentheses(input: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for c in input.chars() {
        match c {
            '(' => stack.push(c),
            ')' => {
                // Pop and check if the popped element is '(', or if stack was empty
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            _ => continue, // Ignore non-parenthesis characters
        }
    }

    // Stack must be empty for balanced parentheses
    stack.is_empty()
}

// Checks if every '(' has a matching ')' and every '[' has a matching ']'.
// Uses a single stack, which correctly identifies mismatches (e.g., "([)]" is invalid
// due to improper nesting and is rejected because the stack pops '[' when expecting '(').
// The single stack cannot robustly handle all interleaved cases due to LIFO limitations.
// Returns true if stack is empty and no mismatches occur, false otherwise.
// Example: "( [] )" → true, "([)]" → false, "([[" → false
//
// `check_parentheses_and_brackets("([)]")` (invalid):
// Initial: stack = []
//   [ ]
// Step 1: Read '(' → stack.push('(')
//   [ ( ] ← Push arrow
// Step 2: Read '[' → stack.push('[')
//   [ ( , [ ] ← Push arrow
// Step 3: Read ')' → stack.pop() (expect '(', get '[' → mismatch)
//   [ ( ] ← Pop arrow (popped '[' ≠ '(')
fn check_parentheses_and_brackets(input: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for c in input.chars() {
        match c {
            '(' | '[' => stack.push(c), // Push opening symbols
            ')' => {
                // Pop and check for matching '('
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                // Pop and check for matching '['
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            _ => continue,
        }
    }

    // Stack must be empty for valid string, but this doesn't guarantee correct nesting
    stack.is_empty()
}

fn main() {
    let tests_parentheses = vec!["(())", "()()", "(()", "())", "a(b)c"];

    println!("Testing parentheses-only checker:");
    for test in tests_parentheses {
        println!("Input: {} -> Balanced: {}", test, check_parentheses(test));
    }

    let tests_both = vec![
        "([)]", "(())[]", "([]]", "[(])", "a(b)[c]",
    ];

    println!("\nTesting parentheses and brackets checker (limited by single stack):");
    for test in tests_both {
        println!(
            "Input: {} -> Balanced: {}",
            test,
            check_parentheses_and_brackets(test)
        );
    }

    println!("\nWhy this language is not Turing complete:");
    println!("- The language uses a single stack, equivalent to a pushdown automaton (PDA).");
    println!("- It can check balanced parentheses (a context-free language).");
    println!("- It cannot reliably check interleaved parentheses and brackets (e.g., '([)]').");
    println!(
        "- A single stack cannot track two independent symbol types without losing information."
    );
    println!(
        "- Turing completeness requires simulating a Turing machine (e.g., via two stacks or random access memory)."
    );
    println!(
        "- Since this language is limited to one stack, it cannot compute all Turing-computable functions."
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parentheses() {
        assert_eq!(check_parentheses("(())"), true);
        assert_eq!(check_parentheses("()()"), true);
        assert_eq!(check_parentheses("(()"), false);
        assert_eq!(check_parentheses("())"), false);
        assert_eq!(check_parentheses("a(b)c"), true);
    }

    #[test]
    fn test_parentheses_and_brackets() {
        assert_eq!(check_parentheses_and_brackets("(())[]"), true);
        assert_eq!(check_parentheses_and_brackets("([]]"), false);
        assert_eq!(check_parentheses_and_brackets("a(b)[c]"), true);
        assert_eq!(check_parentheses_and_brackets("([)]"), false);
    }
}
