/*
 * =====================================================================================
 *
 *       Filename:  ex1-16.cpp
 *
 *    Description:  
 *
 *        Version:  1.0
 *        Created:  01/25/2021 08:34:11 PM
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
    int result = 0, x = 0;

    while (cin >> x)
        result += x;

    cout << result << endl;

    return 0;
}
