/*
 * =====================================================================================
 *
 *       Filename:  ex1-9.cpp
 *
 *    Description:  
 *
 *        Version:  1.0
 *        Created:  01/25/2021 07:10:05 PM
 *       Revision:  none
 *       Compiler:  gcc
 *
 *         Author:  Ji-In Moon (jmoon), jmoon@jiinmoon.com
 *        Company:  
 *
 * =====================================================================================
 */

#include <iostream>

using namespace std;

int main()
{
    int x = 50, result = 0;

    while (x++ <= 100)
        result += x;

    cout << result << endl;

    return 0;
}
