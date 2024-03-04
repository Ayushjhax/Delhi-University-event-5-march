//HW
function greet() {
  console.log("Hello, World!");
}

greet();

//Data Types

let myString: string = "This is a string";
let myNumber: number = 42;
let myBoolean: boolean = true;
let myAny: any = "Or it can be anything";

console.log(myString, myNumber, myBoolean, myAny);

//Functions

function addNumbers(a: number, b: number): number {
  return a + b;
}

let sum = addNumbers(5, 10);
console.log("Sum:", sum);

//Loops

for (let i = 0; i < 5; i++) {
  console.log("Iteration:", i);
}

let count = 0;
while (count < 10) {
  console.log("Count:", count);
  count++;
}

//If-Else

let age = 25;

if (age >= 18) {
  console.log("You are eligible to vote.");
} else {
  console.log("You are not eligible to vote.");
}
