# Sorting Algorithms

This folder contains various sorting algorithms implemented in Rust.

Here is a summary of the characteristics and performance of each algorithm:

| Algorithm      | Stable | In-Place | Best Case | Average Case | Worst Case | Visualization |
| -------------- | ------ | -------- | --------- | ------------ | ---------- | ------------- |
| Bubble Sort    | Yes    | Yes      | O(n)      | O(n^2)       | O(n^2)     | ![Bubble Sort](/pics/bubble_sort.gif) |
| Selection Sort | No     | Yes      | O(n^2)    | O(n^2)       | O(n^2)     | ![Selection Sort](/pics/selection_sort.gif) |
| Insertion Sort | Yes    | Yes      | O(n)      | O(n^2)       | O(n^2)     | ![Insertion Sort](/pics/insertion_sort.gif) |
| Shell Sort     | No     | Yes      | O(n log n)| Depends on gap sequence | O(n^2) | ![Shell Sort](/pics/shell_sort.gif) |
| Merge Sort     | Yes    | No       | O(n log n)| O(n log n)   | O(n log n) | ![Merge Sort](/pics/merge_sort.gif) |
| Quick Sort     | No     | Yes      | O(n log n)| O(n log n)   | O(n^2)     | ![Quick Sort](/pics/merge_sort.gif) |
| Heap Sort      | No     | Yes      | O(n log n)| O(n log n)   | O(n log n) | ![Heap Sort](/pics/heap_sort.gif) |

Please note that the actual performance can vary depending on the specific details of the implementation and the nature of the input data.
