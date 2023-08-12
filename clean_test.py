from os import popen
from colorama import Fore, Style
from re import compile
from subprocess import run
from json import dump

TITLE = "Calculating success rate"
NB_TESTS = 4

test_data = []
regex_result = compile("test result: (?P<test>[a-zA-Z]+). (?P<passed>[0-9]+) passed; (?P<failed>[0-9]+) failed; 0 ignored; 0 measured; 0 filtered out; finished in (?P<timepass>[0-9.]+s)")
regex_test = compile("---- (?P<test_name>test_[a-z0-9]+) stdout ----")
regex_element = compile("Element { category: (?P<category>[a-zA-Z]+), value: \"(?P<data>.*)\"")

def get_information(data : dict, usefull_data : dict):
    r = data["rigth"].removeprefix("Elements { elements: [").removesuffix("] }'").split("}, ")
    l = data["left"].removeprefix("Elements { elements: [").removesuffix("] }'").split("}, ")
    diff = {}
    for line in r:
        if not regex_element.match(line):
            continue
        tmp_dic = regex_element.search(line).groupdict()
        if tmp_dic["category"] == "FileName":
            continue
        if tmp_dic["category"] not in diff:
            diff[tmp_dic["category"]] = []
        diff[tmp_dic["category"]].append(tmp_dic["data"])

    for line in l:
        if not regex_element.match(line):
            continue
        tmp_dic = regex_element.search(line).groupdict()
        if tmp_dic["category"] == "FileName":
            continue
        if tmp_dic["category"] not in diff:
            diff[tmp_dic["category"]] = []
        diff[tmp_dic["category"]].append(tmp_dic["data"])


    usefull_data["diff"][data["name"]] = diff
    return usefull_data


for i in range(NB_TESTS):
    test_todo = "real_data_00"+str(i+1)
    clean_data = []
    raw_data = popen("cargo test --test "+test_todo).read().split('\n')
    for d in raw_data: 
        if len(d) ==0:
            continue
        clean_data.append(d) 
    info = regex_result.search(clean_data[-1]).groupdict()
    
    test_file_data = {
        "test_file" : test_todo,
        "id" : "00"+str(i+1),
        "status" : info["test"],
        "passed" : int(info["passed"]),
        "failed" : int(info["failed"]),
        "total" : int(info["passed"]) + int(info["failed"]),
        "time" : info["timepass"],
        "diff" : {}
    }

    index = 0

    while index +1 < len(raw_data):
        if regex_test.match(raw_data[index]):
            break
        index = index +1

    if regex_test.match(raw_data[index]):
        tmp_data = {}

        while index < len(raw_data)-1:
            if len(raw_data[index]) ==0:
                index = index +1 
                continue
            if " right: `" in raw_data[index]:
                data = raw_data[index].split('`')[1]
                tmp_data["rigth"] = data
                test_file_data = get_information(tmp_data, test_file_data)
                tmp_data = {}
            if "  left: `" in raw_data[index]:
                data = raw_data[index].split('`')[1]
                tmp_data["left"] = data
            if regex_test.match(raw_data[index]):
                tmp_data["name"] = regex_test.search(raw_data[index]).groupdict()['test_name']

            index = index +1 

    test_data.append(test_file_data)

run("clear")

print(Fore.WHITE+"#"*len(TITLE)+"\n"+Fore.YELLOW+TITLE+Fore.WHITE+"\n"+"#"*len(TITLE))
print()
print(Fore.CYAN+"Exporting json data")
output_file = open("test_result.json",'w+')
dump(test_data, output_file,indent=5)
print(Fore.GREEN+"File save successfully")
print()

TOTAL = 0
PASSED = 0

for data in test_data:
    printable = Fore.RESET+"[{id}] ["
    if "OK" in data["status"].upper():
        printable = printable +Fore.LIGHTGREEN_EX+"OK"
    else:
        printable = printable +Fore.RED+"KO"
    printable = printable+Fore.RESET+"] ["+Fore.YELLOW+"{passed}"+Fore.RESET+"\t/{total}]"
    TOTAL = TOTAL + data["total"]
    PASSED = PASSED + data["passed"]
    print(str(printable).format(id=data["id"],status=data["status"],passed=data["passed"],total=data["total"]))

success_rate = round(100*PASSED/TOTAL)
printable = "\n"+Fore.RESET + "#" * len(TITLE) + "\nTest sucess rate :[" + Fore.MAGENTA  +"{rate}"+Fore.RESET+" %]\n"+ Fore.RESET +"#" * len(TITLE)
print(str(printable).format(rate = success_rate))