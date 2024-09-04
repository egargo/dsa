import java.util.Arrays;

public class BubbleSort {
    public static void main(String[] args) {
        int[] numbers = { 5, 4, 3, 2, 1, 10, 9, 8, 7, 6, 12 };

        System.out.println(Arrays.toString(sort(numbers)));
    }

    public static int[] sort(int[] numbers) {
        for (int i = 0; i < numbers.length - 1; i++) {
            for (int j = 0; j < numbers.length - 1; j++) {
                if (numbers[j] > numbers[j + 1]) {
                    int temp = numbers[j];
                    numbers[j] = numbers[j + 1];
                    numbers[j + 1] = temp;
                }
            }
        }

        return numbers;
    }
}
