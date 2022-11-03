#include <iostream>
#include <fstream>
#include <vector>
#include <valarray>

using namespace std;

int main() {
	string filename = "../puzzle/1_input.txt";
	ifstream ifs(filename);
	if (!ifs.is_open()) {
		cerr << "failed to open " << filename << endl;
		return 1;
	}

	vector<int> data;
	int measure;
	while (ifs >> measure) {
		data.push_back(measure);
	}
	valarray<int> measures(data.data(), data.size());
	int window = 3;
	size_t end = measures.size() - window;

	int count = 0;
	int prev = -1;
	for (size_t i = 0; i <= end; ++i) {
		int next = valarray(measures[slice(i, window, 1)]).sum();
		if (prev != -1 && prev < next) {
			count++;
		}
		prev = next;
	}

	cout << "Answer: " << count << endl;

	return 0;
}