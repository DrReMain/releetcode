int finalValueAfterOperations(char **operations, int operationsSize)
{
    int result = 0;

    for (int i = 0; i < operationsSize; i++)
    {
        result += ',' - operations[i][1];
    }

    return result;
}