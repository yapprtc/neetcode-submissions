class TimeMap {
public:
    TimeMap() {
        
    }
    
    void set(string key, string value, int timestamp) {
        store_[key].push_back({timestamp, value});
    }
    
    string get(string key, int timestamp) {
        auto it = std::upper_bound(store_[key].begin(), store_[key].end(), timestamp, [](int t, const auto& v) {
            return t < v.first;
        });

        return it == store_[key].begin() ? "" : std::prev(it)->second;
    }
private:
    unordered_map<string, vector<pair<int, string>>> store_;
};
