#include <map>
#include <string>
#include <cstdio>
#include <vector>
#include <cstring>
#include <fstream>
#include <sstream>
#include <cstdarg>
using namespace std;

int pout(const string& command)
{
	FILE* p = popen(command.c_str(), "w");
	return pclose(p);
}

void addComp(map<string, vector<string>>& m, const string& index, const int count, ...)
{
	vector<string> t;
	va_list args;
	va_start(args, count);
	for(int i = 0; i < count; i++) {t.push_back(va_arg(args, string));}
	va_end(args);
	m[index] = t;
}

bool executeCommands(const vector<string>& commands)
{
	stringstream buffer;
	fstream file;
	try
	{
		const int len = commands.size();
		for(int i = 0; i < len; i++) {buffer << "command: " << commands[i] << " -> " << to_string(pout(commands[i])) << endl;}
		file.open("res.txt", ios::out|ios::app);
		file << buffer.str() << endl;
		file.close();
		return 1;
	} catch(exception& e)
	{
		buffer.str(string());
		file.open("error.log", ios::out|ios::app);
		buffer << "error executing commands:" << endl;
		for(int i = 0; i < commands.size(); i++) {buffer << "\t" << commands[i] << endl;}
		file << buffer.str() << endl;
		file.close();
	}
	return 0;
}

int main(const int argc, const char** argv)
{
	map<string, vector<string>> comp;
	addComp(
		comp,
		"password-db",
		1,
		"cd password-db && cargo build"
	);
	return 0;
}
