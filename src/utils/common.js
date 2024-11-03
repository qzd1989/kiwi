export class Stack {
  constructor(limit) {
    this.stack = [];
    this.limit = limit;
  }
  push(element) {
    if (this.stack.length >= this.limit) {
      this.stack.shift();
    }
    this.stack.push(element);
  }
  stack() {
    return this.stack;
  }
}
