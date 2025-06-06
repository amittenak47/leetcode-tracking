// Last updated: 6/6/2025, 3:31:14 PM
class Solution {
public:
    long long smallestNumber(long long num) {
        if (num == 0) return 0;
        
        long long x = num, remainder = 0;
        map<int,int> hash;
        while(x != 0) {
            remainder = x % 10;
            x /= 10;
            hash[remainder]++;
        }
        
        map<int,int>::iterator it = hash.begin();
        vector<long long> arrayOfNumbers;

        while (it->first == 0) it++;
        arrayOfNumbers.push_back(it->first);
        it->second--;
        it = hash.begin();
        
        while (it != hash.end()) {
            while (it->second > 0) {
                arrayOfNumbers.push_back(abs(it->first));                
                it->second--;
            }
            it++;
        }

        std::ostringstream os;
        for (long long i: arrayOfNumbers) os << i;

        return stoll(os.str());

        // while (it->first==0) it++;
        // min=min*10+(it->first);
        // it->second--;
        // it=hash.begin();
        
        // while(it != hash.end()) {
        //     if (it->second>0) {
        //         min=min*10+(it->first);
        //         it->second--;
        //     }
        //     else
        //         it++;
        // }
        // return min;        
    }
};