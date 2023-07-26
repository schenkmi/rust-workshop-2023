#include <stdint.h>
#include <stdio.h>

#include "rust-lib.h"

int main(int argc, char *argv[])
{
    const uint32_t COUNT = 20;
    uint32_t buffer[COUNT];

    size_t actual_count = get_prime_numbers(buffer, 2, COUNT);
    for (size_t i = 0; i < actual_count; i++)
    {
        printf("%u\n", buffer[i]);
    }
}
