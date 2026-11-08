#include <iostream>
#include <vector>
using namespace std;
 
 
int main(){
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
 
    int T; 
    cin >> T;
    while(T--){
        int n;
        cin >> n;
        vector<int> a(n+1);
        for(int i = 1; i <= n; i++){
            cin >> a[i];
        }
 
        
        vector<int> m(n, -1);
        bool liar = false;
 
        
        for(int i = 1; i <= n; i++){
            if(a[i] == 0){
                if(i > 1){
                    
                    if(m[i-1] == 0){ liar = true; break; }
                    m[i-1] = 1;
                }
                if(i < n){
                    
                    if(m[i] == 1){ liar = true; break; }
                    m[i] = 0;
                }
            }
        }
        if(liar){
            cout << "YES
";
            continue;
        }
 
        
        for(int i = 1; i <= n; i++){
            if(a[i] == 1){
                bool already_win = false;
                
                if(i > 1 && m[i-1] == 0) already_win = true;
                if(i < n && m[i] == 1)  already_win = true;
                if(already_win) 
                    continue;
 
                
                bool assigned = false;
                if(i > 1 && m[i-1] == -1){
                    m[i-1] = 0;  
                    assigned = true;
                }
                else if(i < n && m[i] == -1){
                    m[i] = 1;    
                    assigned = true;
                }
 
                if(!assigned){
                    
                    liar = true;
                    break;
                }
            }
        }
 
        cout << (liar ? "YES
" : "NO
");
    }
 
    return 0;
}