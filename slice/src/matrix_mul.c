// #include <stdio.h>
#include <stdlib.h>
#include <assert.h>

int *matrix_mul(int *a ,int *b, size_t a_r, size_t a_c, size_t b_r, size_t b_c){
  int * c = (int *)calloc(a_r * b_c, sizeof(int));
  assert(a_c == b_r);
  for(int i = 0; i < a_r; i++){
    for(int j = 0; j < b_c; j++){
      for(int k = 0;k < a_c;k++){
        c[(i * b_c) + j] += a[(i * a_c) + k] * b[ k * b_c + j];
      }
    }
  }
  return c;
}

int *c_alloc(int size){
  int * buffer = (int *)malloc(size * sizeof(int));
  return buffer;
}

void c_out_of_bound() {
    int array[5] = {1, 2, 3, 4, 5};
    int value = array[5];
    printf("Value: %d\n", value);
    int *a = (int*) malloc(3 * sizeof(int));
    printf("Value: %d\n", a[3]);
    // free(a);
}

void out_of_bound(int* a){
  int c = a[3];
  printf("Value: %d\n", a[3]);
}

void c_free(int* a){
  free(a);
}

int *allocate_uninitialized_array(int size) {
    return (int*)malloc(size * sizeof(int));
}

void c_uninitialized_read(){

}
// int main(){
//   int a[6] = {1, 2, 3, 4, 5, 6};
//   int b[9] = {1, 2, 3, 4, 5, 6, 7, 8, 9};
//   int *c = matrix_mul(a, b, 2, 3, 3, 3);
//   for(int i = 0; i < 2; i++){
//     for(int j = 0; j < 3;j++){
//       printf("%d ", c[i * 3 + j]);
//     }
//     printf("\n");
//   }
//   return 0;
// }


