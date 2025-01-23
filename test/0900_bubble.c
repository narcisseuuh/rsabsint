// Example from Cousot Habwachs POPL 1978

// Bubble sort, where array  operations are abstracted away
// to get a non-deterministic scalar program

// expected results:
// - with polyhedra, no assertion failure (proof of absence of array overflow)
// - with intervals, assertion failure

{
  int B;
  int J;
  int T;
  int N;

  N = rand(0,50);  // array size
  
  B = N;
  while (B >= 1) {
    J = 1;
    T = 0;
    while (J <= B-1) {
      
      // array bound-check 
      assert(1 <= J && J <= N);
      assert(1 <= (J+1) && (J+1) <= N);
      
      // the non-deterministic test models comparing TAB[J] and TAB[J+1]
      if (rand(0,1)==0) {
        // where, we would exchange TAB[J] and TAB[J+1];
        T = J;
      }
      
      J = J + 1;
    }
    B = T;
  }
}
