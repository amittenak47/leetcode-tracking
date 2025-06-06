// Last updated: 6/6/2025, 3:31:59 PM
class Solution {
public:
    vector<string> fizzBuzz(int n) {
        vector<string> ans;
        
        vector<int> range(n);
        iota(range.begin(), range.end(), 1);
        for (auto num : range) {
            string element = ((num % 15 == 0)) ? "FizzBuzz" : (num % 3 == 0) ? "Fizz" : (num % 5 == 0) ? "Buzz" : to_string(num);
            ans.emplace_back(element);
        }

        return ans;
    }        
};