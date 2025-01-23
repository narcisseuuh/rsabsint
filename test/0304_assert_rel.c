{
  int x;
  int y;
  if (rand(0,1)==0) { x = 1; y = 1; }
  else { x = 3 ; y = 3; }
  assert(x==y);
  print(x,y);
}
