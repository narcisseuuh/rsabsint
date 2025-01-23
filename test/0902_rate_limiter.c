// Example from Miné HOSC 2006

// Rate limiter: at each loop iteration, a new input is fetched (X) and
// a new output (Y) is computed; Y tries to follow X but is limited to
// change no more that a given slope (D) in absolute value

// To prove that the assertion holds, this version needs the polyhedra
// domain and an unrolling factor of at least 6

{
  int X;  // input
  int Y;  // output
  int S;  // last output
  int D;  // maximum slope;
  int R; // current slope

  Y = 0;
  while (rand(0,1)==1) {
    X = rand(-128,128);
    D = rand(0,16);
    S = Y;
    R = X - S; // current slope
    Y = X;
    if (R <= -D)     Y = S - D; // slope too small
    else if (R >= D) Y = S + D; // slope too large
  }
  
  assert(Y >= -128 && Y <= 128);
}
