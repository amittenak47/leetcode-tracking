// Last updated: 6/6/2025, 3:32:08 PM
class MovingAverage {
public:
    int windowSize;
    double windowSum;
    queue<int> queue;

    MovingAverage(int size):windowSize(size), windowSum(0) {}
    
    double next(int val) {
        queue.push(val);
        windowSum += val;
        int queueSize = queue.size();
        
        if (queueSize <= windowSize) { return windowSum / queueSize; } 
        else {
            windowSum -= queue.front();
            queue.pop();
            return windowSum * 1.0 / windowSize;
        }      
    }
};

/**
 * Your MovingAverage object will be instantiated and called as such:
 * MovingAverage* obj = new MovingAverage(size);
 * double param_1 = obj->next(val);
 */
