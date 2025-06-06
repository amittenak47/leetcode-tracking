// Last updated: 6/6/2025, 3:31:56 PM
class Solution {
public:
    bool flag = false;
    bool checkInclusion(string s1, string s2) {
        
        // Brute Force O(n!)
        // permute(s1, s2, 0);
        // return flag;


        // Sorting O(l1*log l1 + (l2 - l1)*((l2 - l1)log(l2 - l1)))
        // if (s1.compare(s2) == 0) return true; 
        // if (s1.length() > s2.length()) return false; 
        
        // s1 = sort(s1); // sort s1

        // // iterate (s2-s1 length) times comparing substring of s1 length
        // for (int i = 0; i <= s2.length() - s1.length(); i++) { 
        //     if (s1.compare(sort(s2.substr(i, s1.length()))) == 0) return true;
        // }
        // return false;

        // Sliding Window Optimized O(l1 + (l2 - l1))
        
        if (s1.length() > s2.length()) return false;

        int s1map[26] = {}, s2map[26] = {};
        for (int i = 0; i < s1.length(); i++) { // O(l1) fill freq maps
            s1map[s1.at(i) - 'a']++;
            s2map[s2.at(i) - 'a']++;
        }

        int count = 0;
        for (int i = 0; i < 26; i++) { if (s1map[i] == s2map[i]) { count++; } } // O(l1) s2/s1 char freq of all 26 char

        for (int i = 0; i < s2.length() - s1.length(); i++) { // O(l2 - l1) i: starting (s2 - s1)
            
            if (count == 26) { return true; } // if s1 == s2 for all 26 char freq, return true 

            int r = s2.at(i + s1.length()) - 'a', l = s2.at(i) - 'a'; // get left/right window char from s2 string
            
            s2map[r]++; // increment frequency of right char
            if (s2map[r] == s1map[r]) { count++; } // count++ if freq matches after incrementing right char of s2 freq
            else if (s2map[r] == s1map[r] + 1) { count--; } // count-- if freq mismatch after incrementing right char of s2 freq

            s2map[l]--; // increment frequency of left char
            if (s2map[l] == s1map[l]) { count++; } // count++ if freq matches after incrementing left char of s2 freq
            else if (s2map[l] == s1map[l] - 1) { count--; } // count-- if freq mismatch after incrementing left char of s2 freq
        }
        return count == 26; // return result of s1 == s2 for all 26 char freq

        // Array O(l1 + 26 * l1 (l2 - l1))
        // 
        // if (s1.length() > s2.length()) return false;
        
        // unordered_map<char, int> s1map; // O(l1) make s1 map
        // for (int i = 0; i < s1.length(); i++) {
        //     char letter = s1.at(i); // get letter at index i 
        //     int frequency = (s1map[letter]) ? s1map[letter] : 0; // get current frequency or set 0 as frequency  
        //     s1map[letter] = frequency + 1; // store the letter with +1 frequency
        // }

        // for (int i = 0; i <= s2.length() - s1.length(); i++) { 
        //     // O(l2 - l1) Make map of freq for all permutations, i: starting (s2 - s1)
            
        //     unordered_map<char, int> s2map; // make s2map
        //     for (int j = 0; j < s1.length(); j++) { // O(l1) +j: [0, s1 length]
        //         char letter = s2.at(i + j); // get letter at index i + j , 
        //         int frequency = (s2map[letter]) ? s2map[letter] : 0; // get current frequency or set 0 as frequency  
        //         s2map[letter] = frequency + 1; // store the letter with +1 frequency
        //     }
            
        //     if (matches(s1map, s2map)) return true; // O(26) if s1map == s2map return true
        // }
        // return false;

        // Sliding Window O(l1 + 26(l2 - l1))  
        // if (s1.length() > s2.length()) return false;
        
        // int s1map[26], s2map[26];
        
        // for (int i = 0; i < s1.length(); i++) { // O(l1)
        //     s1map[s1.at(i) - 'a']++;
        //     s2map[s2.at(i) - 'a']++;
        // }
        // for (int i = 0; i < s2.length() - s1.length(); i++) { // O(l2 -l1)
            
        //     if (matches(s1map, s2map)) return true; // O(26)
            
        //     s2map[s2.at(i + s1.length()) - 'a']++; // *SLIDING WINDOW* add new character at end of range from s2
        //     s2map[s2.at(i) - 'a']--; // remove character from start of range from s2
        // }
        // return matches(s1map, s2map); // O(26)
    }

    string swap(string s, int i0, int i1) {
        if (i0 == i1) return s;
        string s1 = s.substr(0, i0), s2 = s.substr(i0 + 1, i1 - (i0 + 1)), s3 = s.substr(i1, (i1 + 1) - i1);
        return s1 + s.at(i1) + s2 + s.at(i0) + s3;
    }
    
    void permute(string s1, string s2, int l) {
        if (l == s1.length()) {
            if (s2.find(s1, 0) >= 0) flag = true;
        } else {
            for (int i = l; i < s1.length(); i++) {
                s1 = swap(s1, l, i);
                permute(s1, s2, l + 1);
                s1 = swap(s1, l, i);
            }
        }
    }

    bool matches(int s1map[], int s2map[]) {
        for (int i = 0; i < 26; i++) {
            if (s1map[i] != s2map[i]) return false; // check all 26 char of s1map == s2map
        }
        return true;
    }

    bool matches(unordered_map<char, int> s1map, unordered_map<char, int> s2map) {
        
        std::vector<int> keys = getKeys(s1map);
        for (auto key : keys) {
            int frequency = (s2map[key]) ? s2map[key] : -1; // get s2map frequency
            if ((s1map[key] - frequency) != 0) return false; // compare s1map and s2map, returning false if unequal            
        }
        return true;
    }

    vector<int> getKeys(unordered_map<char, int> smap) {
        std::vector<int> keys;
        for (auto it = smap.begin(); it != smap.end(); it++) {
            keys.push_back(it->first);
        }
        return keys;
    }
    
    string sort(string s) {
        string t = s.c_str(); // convert string to array
        std::sort(t.begin(), t.end()); // sort array
        string new_str(t); // construct string from array

        return new_str; // return sorted string
    }
};