#include <iostream>
#include <fstream>
#include <vector>
#include <valarray>
#include <map>
#include <stdexcept>

using namespace std;

enum class Command {Forward, Down, Up};
const map<string, Command> StringToCommands = {
	{"forward", Command::Forward},
	{"down", Command::Down},
	{"up", Command::Up}
};

Command strToCmd(string cmd) {
	auto it = StringToCommands.find(cmd);
	if (it != StringToCommands.end()) {
		return it->second;
	}
	else {
		throw invalid_argument("received invalid command: " + cmd);
	}
}

struct Submarine {
	int hor = 0;
	int depth = 0;

	void forward(int x) {
		hor += x;
	}

	void down(int x) {
		depth += x;
	}

	void up(int x) {
		depth -= x;
	}

	void move(Command cmd, int val) {
		switch (cmd) {
			case Command::Forward:
				this->forward(val);
				break;
			case Command::Down:
				this->down(val);
				break;
			case Command::Up:
				this->up(val);
				break;
		}
	}

	int getPosition() const {
		return hor * depth;
	}
};

int main() {
	string filename = "../2_input.txt";
	ifstream ifs(filename);
	if (!ifs.is_open()) {
		cerr << "failed to open " << filename << endl;
		return 1;
	}

	Submarine sub;
	string cmd;
	int val;
	while (ifs >> cmd >> val) {
		sub.move(strToCmd(cmd), val);
	}

	cout << sub.getPosition() << endl;

}
