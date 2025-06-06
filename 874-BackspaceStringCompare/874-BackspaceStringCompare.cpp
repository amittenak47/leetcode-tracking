// Last updated: 6/6/2025, 3:31:41 PM
class Solution {
public:
    bool backspaceCompare(string s, string t) {
    //     return build(s) == build(t);
    // }

    // string build(string S) {
    //     string ans;
    //     for (char c : S)
    //         if (c != '#') {
    //             ans.push_back(c);
    //         }
    //         else {
    //             ans.pop_back();
    //         }
        
    //     return ans;
    // }
        int i = s.length() - 1, j = t.length() - 1, skipS = 0, skipT = 0;

        while (i >= 0 || j >= 0) { // chars in build(S) or build (T)
            while (i >= 0) { // build(S)
                if (s.at(i) == '#') {skipS++; i--;} // add skip so decrement
                else if (skipS > 0) {skipS--; i--;} // extra skip so decrement
                else break;
            }
            while (j >= 0) { // build(T)
                if (t.at(j) == '#') {skipT++; j--;}
                else if (skipT > 0) {skipT--; j--;}
                else break;
            }
            
            if (i >= 0 && j >= 0 && s.at(i) != t.at(j)) return false; // // char different different, return false
            if ((i >= 0) != (j >= 0)) return false; // no char to compare, return false

            i--; j--; // char same, continue 
        }
        return true;
    }
};