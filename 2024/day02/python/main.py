import os

def get_rules(report):
    unsafe = 0
#create list of integers
    str_list = report.split()
    int_list = list(map(int, str_list))
#iterate line through rules, counting safe and unsafe levels
    if int_list[0] <= int_list[1]:
        for i in range(len(int_list) -1) :
            if int_list[i] >= int_list[i+1] or int_list[i] < (int_list[i+1]-3) or int_list[i] == (int_list[i+1]):
                    unsafe += 1

    elif int_list[0] > int_list[1]:
        for i in range(len(int_list) -1):
            if int_list[i] <= int_list[i+1] or int_list[i] > (int_list[i+1]+3) or int_list[i] == (int_list[i+1]):
                 unsafe += 1
    else:
        unsafe += 1
    return unsafe
    

def main():
    report = ""
    safe_reports = 0
    acceptable_reports = 0
    
    #read lines, one at a time
    with open('../input.txt', 'r') as file:
        for line in file:
            report = line.strip()
            unsafe = get_rules(report)
            if unsafe == 0:
                safe_reports += 1
            if unsafe == 1:
                acceptable_reports += 1
    #part 1 return # of safe lines
    print(f"Total safe reports: {safe_reports}")
    #part 2 disregard a single unsafe level.
    acceptable_reports += safe_reports
    print(f"Total safe reports w/ 1 exception: {acceptable_reports}")


main()