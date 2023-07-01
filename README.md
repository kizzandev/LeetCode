# LeetCode

Solutions for LeetCode in C++, Python, and Rust.

A quick note: I'm learning Rust with this.

## Project structure

```mermaid
%%{init: {'flowchart' : {'curve' : 'stepAfter'}}}%%
graph LR;
    ROOT((ROOT))
    ROOT-->C++
    ROOT-->Python
    ROOT-->Rust
    ROOT-->SQL[("&nbsp;&nbsp;SQL&nbsp;&nbsp;")]
    C++-->Problem_x
    Python-->Problem_x
    Rust-->Problem_folder
    Problem_folder-->src
    src-->Problem_x
    SQL-->Problem_y
```

## Gantt Chart

### LeetCode 75

<!-- title : (empyty|active|done) , name , after name , 1d -->
```mermaid
gantt
    section Array/Str
    Merge Strings Alternately               : done,   a0, 2000-01-01, 1d
    Greatest Common Divisor of Strings      : done,   a,  2000-01-01, 1d
    Kids With the Greatest Number of Candies: active, a,  2000-01-01, 1d
    Can Place Flowers                       :         a,  2000-01-01, 1d
    Reverse Vowels of a String              :         a,  2000-01-01, 1d
    Reverse Words in a String               :         a,  2000-01-01, 1d
    Product of Array Except Self            :         a,  2000-01-01, 1d
    Increasing Triplet Subsequence          :         a,  2000-01-01, 1d
    String Compression                      :         a,  2000-01-01, 1d

    section Two Pointers
    Move Zeroes              :         a1, after a0, 1d
    Is Subsequence           :         a , after a0, 1d
    Container With Most Water:         a , after a0, 1d
    Max Number of K-Sum Pairs:         a , after a0, 1d

    section Sliding Window
    Maximum Average Subarray I                             :         a2, after a1, 1d
    Maximum Number of Vowels in a Substring of Given Length:         a , after a1, 1d
    Max Consecutive Ones III                               :         a , after a1, 1d
    Longest Subarray of 1's After Deleting One Element     :         a , after a1, 1d

    section Prefix Sum
    Find the Highest Altitude:         a3, after a2, 1d
    Find Pivot Index         :         a , after a2, 1d
```

### SQL 50

```mermaid
gantt
    section Select
    Recyclable and Low Fat Products: done,   a0, 2000-01-01, 1d
    Find Customer Referee          : done,   a , 2000-01-01, 1d
    Big Countries                  : done,   a , 2000-01-01, 1d
    Article Views I                : active, a , 2000-01-01, 1d
    Invalid Tweets                 :         a , 2000-01-01, 1d
```
