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
        
        if (it != store_[key].end()) {
            if (it != store_[key].begin()) {
                it = std::prev(it);
                //std::cout << "it=(" << it->first << "," << it->second<<"), timestamp=" << timestamp << std::endl;
                if (timestamp >= it->first){
                    return it->second;
                }
            }
        } else if (!store_[key].empty()) {
            it = std::prev(it);
            //std::cout << "prev(it)=(" << it->first << "," << it->second<<"), timestamp=" << timestamp << std::endl;
            if (timestamp >= it->first){
                return it->second;
            }
        } 
        return "";
    }
private:
    unordered_map<string, vector<pair<int, string>>> store_;
};
