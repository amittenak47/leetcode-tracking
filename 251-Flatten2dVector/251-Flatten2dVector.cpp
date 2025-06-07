// Last updated: 6/7/2025, 2:41:54 AM
class Vector2D {
private:
    vector<vector<int>> vector1D;
    int inner = 0;
    int outer = 0;
public:
    Vector2D(vector<vector<int>>& vec) {
        vector1D = vec;
    }

    void advanceToNext() {
        while (outer < vector1D.size() && inner == vector1D[outer].size()) {
            inner = 0;
            outer++;
        }
    }

    int next() {
        if (!hasNext()) return -1;
        return vector1D[outer][inner++];
    }

    bool hasNext() {
        advanceToNext();
        return outer < vector1D.size();
    }
};

/**
 * Your Vector2D object will be instantiated and called as such:
 * Vector2D* obj = new Vector2D(vec);
 * int param_1 = obj->next();
 * bool param_2 = obj->hasNext();
 */