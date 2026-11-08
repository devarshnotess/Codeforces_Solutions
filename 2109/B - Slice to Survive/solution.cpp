#include <iostream>
using namespace std;
 
 
int clog2(long long x) {
    
    int f = 63 - __builtin_clzll(x);
    return ( (1LL<<f) == x ? f : f + 1 );
}
 
int main(){
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
 
    int T; 
    cin >> T;
    while(T--){
        long long n, m, a, b;
        cin >> n >> m >> a >> b;
 
        
        long long r_rem = min(a, n - a + 1);
        long long c_rem = min(b, m - b + 1);
 
        int best_after_row_cut = clog2(r_rem) + clog2(m);
        int best_after_col_cut = clog2(n)     + clog2(c_rem);
 
        
        cout << (1 + min(best_after_row_cut, best_after_col_cut)) << "
";
    }
    return 0;
}