#include <Windows.h>

// put all monitors to sleep (power saving mode)
int main(int argc, char** argv) {
	LRESULT messageResult = SendMessageW(HWND_BROADCAST, WM_SYSCOMMAND, SC_MONITORPOWER, 0x0002 /* POWER OFF */);
	
	// unsuccessfull message processing
	if (messageResult == NULL) { return 1; }
	
	return 0;
}