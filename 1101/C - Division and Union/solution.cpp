#include <bits/stdc++.h>
 
using namespace std;
 
void solve(){
    
    int n;
    cin >> n;
    
    vector<vector<int>> a(n);
    
    for (int i = 0; i < n; i++){
        
        int l, r;
        cin >> l >> r;
        a[i] = {l, r, i};
    } 
 
    sort(a.begin(), a.end()); 
 
    vector<int> ans(n, 2);
    int maximum = a[0][1];
    
    bool okay = 0;
    for (int i = 0; i < n; i++) {
        if (a[i][0] > maximum) {
            okay = 1;
            break;
        }
        ans[a[i][2]] = 1;
        maximum = max(maximum, a[i][1]);
    } 
 
    if (!okay)
        cout << "-1
";
        
    else{
        for (auto i : ans)
            cout << i << ' '; 
        cout << '
';
    }
}
 
 
 
int32_t main(){
    
    ios::sync_with_stdio(NULL);
    cin.tie(NULL);
    cout.tie(NULL);
 
    int t;
    cin >> t;
    
    while (t--) {
        solve();
    }
 
    return 0;
}