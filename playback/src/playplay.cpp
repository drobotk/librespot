#include <stdint.h>

extern "C" {
    int32_t playplay_get_version() {
        return -1;
    }
    void playplay_get_token(uint8_t buffer[16]) {
    }
	void playplay_decrypt(uint8_t key[16], uint8_t fileId[16], uint8_t outputBuffer[16]) {
	}
}