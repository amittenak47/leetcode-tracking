// Last updated: 6/7/2025, 2:41:53 AM
class Solution {
public:
    // bool canAttendMeetings(vector<vector<int>>& intervals) {

    //     if (intervals.empty()) return true;

    //     sort(intervals.begin(), intervals.end()); // N log N 
        
    //     for (size_t i = 0; i < intervals.size() - 1; i++) if (intervals[i][1] > intervals[i + 1][0]) return false; // cant merge end meeting1 > begin meeting2
        
    //     return true;
    // }

    // method checks for overlap between two intervals (interval1 starts ahead of interval2 but ends before interval2 and vice versa)
    bool overlap(vector<int>& interval1, vector<int>& interval2) {
        return interval1[0] >= interval2[0] and interval1[0] < interval2[1] or interval2[0] >= interval1[0] and interval2[0] < interval1[1];
    }
    
    bool canAttendMeetings(vector<vector<int>>& intervals) {

        // brute force each permutation of interval checking for overlap
        for (size_t i = 0; i < intervals.size(); i++) {
            for (size_t j = i + 1; j < intervals.size(); j++) if (overlap(intervals[i], intervals[j])) return false;
        } 
        return true;
    }
};