---
title: C++ Algorithms
date: 2026-01-19
tags: [cpp, algorithms, data-structures]
author: "Anonymous"
---

C++ Algorithms Example
======================

This snippet demonstrates common algorithms and data structures in C++.

.. code-block:: cpp

    #include <iostream>
    #include <vector>
    #include <algorithm>
    #include <string>
    #include <map>
    #include <set>
    #include <queue>
    #include <memory>
    
    // Template function for binary search
    template <typename T>
    int binarySearch(const std::vector<T>& arr, const T& target) {
        int left = 0;
        int right = arr.size() - 1;
        
        while (left <= right) {
            int mid = left + (right - left) / 2;
            
            if (arr[mid] == target) {
                return mid;
            } else if (arr[mid] < target) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        
        return -1; // Not found
    }
    
    // Tree node structure
    struct TreeNode {
        int val;
        std::unique_ptr<TreeNode> left;
        std::unique_ptr<TreeNode> right;
        
        TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    };
    
    // Insert into BST
    std::unique_ptr<TreeNode> insertIntoBST(
        std::unique_ptr<TreeNode> root, 
        int val
    ) {
        if (!root) {
            return std::make_unique<TreeNode>(val);
        }
        
        if (val < root->val) {
            root->left = insertIntoBST(std::move(root->left), val);
        } else {
            root->right = insertIntoBST(std::move(root->right), val);
        }
        
        return root;
    }
    
    // Inorder traversal
    void inorderTraversal(const TreeNode* root, std::vector<int>& result) {
        if (!root) return;
        
        inorderTraversal(root->left.get(), result);
        result.push_back(root->val);
        inorderTraversal(root->right.get(), result);
    }
    
    // Graph class using adjacency list
    class Graph {
    private:
        std::map<int, std::vector<int>> adjList;
        bool directed;
        
    public:
        Graph(bool isDirected = false) : directed(isDirected) {}
        
        void addEdge(int u, int v) {
            adjList[u].push_back(v);
            if (!directed) {
                adjList[v].push_back(u);
            }
        }
        
        // BFS traversal
        std::vector<int> bfs(int start) {
            std::vector<int> result;
            std::queue<int> q;
            std::set<int> visited;
            
            q.push(start);
            visited.insert(start);
            
            while (!q.empty()) {
                int current = q.front();
                q.pop();
                result.push_back(current);
                
                for (int neighbor : adjList[current]) {
                    if (visited.find(neighbor) == visited.end()) {
                        visited.insert(neighbor);
                        q.push(neighbor);
                    }
                }
            }
            
            return result;
        }
        
        // DFS traversal
        void dfsUtil(int node, std::set<int>& visited, std::vector<int>& result) {
            visited.insert(node);
            result.push_back(node);
            
            for (int neighbor : adjList[node]) {
                if (visited.find(neighbor) == visited.end()) {
                    dfsUtil(neighbor, visited, result);
                }
            }
        }
        
        std::vector<int> dfs(int start) {
            std::vector<int> result;
            std::set<int> visited;
            dfsUtil(start, visited, result);
            return result;
        }
    };
    
    int main() {
        std::cout << "=== C++ Algorithms Demo ===\n\n";
        
        // 1. Binary Search
        std::vector<int> sortedArray = {1, 3, 5, 7, 9, 11, 13, 15, 17, 19};
        int target = 7;
        int index = binarySearch(sortedArray, target);
        std::cout << "Binary Search: Found " << target << " at index " << index << "\n\n";
        
        // 2. BST operations
        std::unique_ptr<TreeNode> bstRoot;
        std::vector<int> values = {5, 3, 7, 2, 4, 6, 8};
        
        for (int val : values) {
            bstRoot = insertIntoBST(std::move(bstRoot), val);
        }
        
        std::vector<int> inorderResult;
        inorderTraversal(bstRoot.get(), inorderResult);
        
        std::cout << "BST Inorder Traversal: ";
        for (int val : inorderResult) {
            std::cout << val << " ";
        }
        std::cout << "\n\n";
        
        // 3. Graph traversal
        Graph graph(false); // Undirected graph
        graph.addEdge(0, 1);
        graph.addEdge(0, 2);
        graph.addEdge(1, 3);
        graph.addEdge(2, 4);
        graph.addEdge(3, 5);
        graph.addEdge(4, 5);
        
        std::vector<int> bfsResult = graph.bfs(0);
        std::cout << "Graph BFS from node 0: ";
        for (int node : bfsResult) {
            std::cout << node << " ";
        }
        std::cout << "\n";
        
        std::vector<int> dfsResult = graph.dfs(0);
        std::cout << "Graph DFS from node 0: ";
        for (int node : dfsResult) {
            std::cout << node << " ";
        }
        std::cout << "\n";
        
        // 4. STL algorithms
        std::vector<int> numbers = {5, 2, 8, 1, 9, 3, 7, 4, 6};
        
        // Sort
        std::sort(numbers.begin(), numbers.end());
        std::cout << "\nSorted numbers: ";
        for (int num : numbers) {
            std::cout << num << " ";
        }
        std::cout << "\n";
        
        // Find
        auto it = std::find(numbers.begin(), numbers.end(), 7);
        if (it != numbers.end()) {
            std::cout << "Found 7 in the vector\n";
        }
        
        // Count
        int count = std::count(numbers.begin(), numbers.end(), 5);
        std::cout << "Count of 5: " << count << "\n";
        
        // Accumulate
        int sum = std::accumulate(numbers.begin(), numbers.end(), 0);
        std::cout << "Sum of all numbers: " << sum << "\n";
        
        return 0;
    }
