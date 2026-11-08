#include <iostream>
using namespace std;
 
int main() {
    int t;
    cin >> t;
    
    while (t--) {
        int n;
        cin >> n;
        
        int x = 0; 
        int i = 1; 
        while (true) {
            int move = 2 * i - 1;
            if (i % 2 == 1) {
                x -= move; 
            } else {
                x += move; 
            }
            if (abs(x) > n) {
                if (i % 2 == 1)
                    cout << "Sakurako" << endl;
                else
                    cout << "Kosuke" << endl;
                break;
            }
            ++i;
        }
    }
 
    return 0;
}