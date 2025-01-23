// Example from Cousot Habwachs POPL 1978

// Heap sort, where array  operations are abstracted away
// to get a non-deterministic scalar program

// expected results: with polyhedra, no assertion failure

{
  int L;
  int R;
  int I;
  int J;
  int N;
  
  N = rand(1,50);  // array size
  
  L = N/2 + 1;
  R = N;
  
  if (L >= 2) {
    L = L - 1; 
    // model the assignment "K = TAB[L]"
    assert(1 <= L && L <= N);
  }
  else {
    // model the assignments "K = TAB[R]; TAB[R] = TAB[1]"
    assert(1 <= R && R <= N);
    assert(1 <= 1 && 1 <= N);
    R = R - 1;
  }
  
  while (R >= 2) {
    I = L;
    J = 2*I;
    
    while (J <= R && rand(0,1)==0) {
      if (J <= R-1) {
        // model the comparison "TAB[J] < TAB[J+1]"
        assert(1 <= J && J <= N);
        assert(1 <= (J+1) && (J+1) <= N);
        if (rand(0,1)==0) { J = J + 1; }
      }
      
      // model the comparison "K < TAB[J]"
      assert(1 <= J && J <= N);
      if (rand(0,1)==0) {
        // model the assignment "TAB[I] = TAB[J]"
        assert(1 <= I && I <= N);
        assert(1 <= J && J <= N);   
        I = J;
        J = 2*J;
      }
    }
    
    // model the assignment "TAB[I] = K"
    assert(1 <= I && I <= N);
    
    if (L >= 2) {
      L = L - 1; 
      // model the assignment "K = TAB[L]"
      assert(1 <= L && L <= N);
    }
    else {
      // model the assignments "K = TAB[R]; TAB[R] = TAB[1]"
      assert(1 <= R && R <= N);
      assert(1 <= 1 && 1 <= N);
      R = R - 1;
    }
    
    // model the assignment "TAB[1] = K"
    assert(1 <= 1 && 1 <= N);
  }
}

