def add_one_to_list(lst):
    for i in range(len(lst)):
        lst[i] += 1

lst = [1, 2, 3]
add_one_to_list(lst)
print(lst)

# This will modify the list outside of the function
lst[0] += 1
print(lst)