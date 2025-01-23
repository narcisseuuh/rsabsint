// This version is similar to the previous one, but the assertion is checked
// inside the loop
// No unrolling is necessary in this version!

{
  int X;  // input
  int Y;  // output
  int S;  // last output
  int D;  // maximum slope;
  int R;  // current slope
  
  Y = 0;
  while (rand(0,1)==1) {
    X = rand(-128,128);
    D = rand(0,16);
    S = Y;
    R = X - S;
    Y = X;
    if (R <= -D)     Y = S - D; // slope too small
    else if (R >= D) Y = S + D; // slope too large
    
    assert(Y >= -128 && Y <= 128);
  }
}
