// Last updated: 6/6/2025, 3:31:42 PM
class Bucket {
    
    private:

        std::vector<pair<int, int>> bucket;

    public:

        int get(int key) {
            for (pair<int, int> pair : (this)->bucket) {
              if (pair.first == key) return pair.second;
            }
            return -1;
        }

        void update(int key, int value) {
            bool found = false;
            for (pair<int, int> &pair : (this)->bucket) {
                if (pair.first == key) {
                    pair.second = value;
                    found = true;
                }
            }
            if (!found) (this)->bucket.push_back(pair<int, int>(key, value));
        }

        void remove(int key) {
            for (int i = 0; i < (this)->bucket.size(); ++i) {
                if ((this)->bucket[i].first == key) { 
                    (this)->bucket.erase((this)->bucket.begin() + i); 
                    break; 
                }
            }
        }
};

class MyHashMap {
    private:
        const static int key_space = 2069;
        const static int mult = 12582917;
        vector<Bucket> hash_table;

        int hash(int key) { return (int)((long) key * mult % key_space); }

    public:

        MyHashMap() {
            for (int i = 0; i < (this)->key_space; ++i) {
                Bucket bucket;
                hash_table.push_back(bucket);
            }
        }

        void put(int key, int value) {
            int hash_key = hash(key);
            (this)->hash_table.at(hash_key).update(key, value);
        }

        int get(int key) {
            int hash_key = hash(key);
            return (this)->hash_table.at(hash_key).get(key);
        }

        void remove(int key) {
            int hash_key = hash(key);
            (this)->hash_table.at(hash_key).remove(key);
        }
};
