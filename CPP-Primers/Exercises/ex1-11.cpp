/*
 * =====================================================================================
 *
 *       Filename:  ex1-11.cpp
 *
 *    Description:  
 *
 *        Version:  1.0
 *        Created:  01/25/2021 07:11:34 PM
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
    int beg = 0, end = 0, result = 0;
    cout << "Enter two values to sum in range (x .. y):" << endl;

    cin >> beg >> end;

    if (beg >= end)
    {
        cout << "Range error." << endl;
        return 0;
    }
    cout << "Range sum from " << beg << " to " << end << " is ";

    while (beg < end + 1)
        result += beg++;

    cout << result << endl;

    return 0;
}
