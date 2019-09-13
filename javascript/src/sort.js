function swap(items, leftIndex, rightIndex) {
  var temp = items[leftIndex];
  items[leftIndex] = items[rightIndex];
  items[rightIndex] = temp;
}

function partition(items, left, right) {
  var pivot = items[Math.floor((right + left) / 2)], //middle element
    i = left, //left pointer
    j = right; //right pointer
  while (i <= j) {
    while (items[i] < pivot) {
      i++;
    }
    while (items[j] > pivot) {
      j--;
    }
    if (i <= j) {
      swap(items, i, j); //sawpping two elements
      i++;
      j--;
    }
  }
  return i;
}

function quickSort(items, left, right) {
  var index;
  if (items.length > 1) {
    index = partition(items, left, right); //index returned from partition
    if (left < index - 1) { //more elements on the left side of the pivot
      quickSort(items, left, index - 1);
    }
    if (index < right) { //more elements on the right side of the pivot
      quickSort(items, index, right);
    }
  }
  return items;
}

function quickSort2(array) {
  if (array.length < 2) {
    return array
  }
  const chosenIndex = array.length - 1
  const chosen = array[chosenIndex]
  const a = []
  const b = []
  for (let i = 0; i < chosenIndex; i++) {
    const temp = array[i]
    temp < chosen ? a.push(temp) : b.push(temp)
  }

  const output = [...quickSort(a), chosen, ...quickSort(b)]
  console.log(output.join(' '))
  return output
}

function bubbleSort(array, pointer = array.length - 1) {
  if (pointer === 0) {
    return array;
  }

  for (let i = 0; i < pointer; i++) {
    if (array[i] > array[i + 1]) {
      swap(array, i, i + 1);
    }
  }
  // Recursive call on smaller portion of the array
  return bubbleSort(array, pointer - 1);
}

function insertionSort(arr, i = arr.length) {
  //if index is less than 1 then return
  if (i <= 1) {
    return;
  }

  //Recursively call the same function
  insertionSort(arr, i - 1);

  let key = arr[i - 1];
  let j = i - 2;

  //Sort the array
  while (j >= 0 && arr[j] > key) {
    arr[j + 1] = arr[j];
    j--;
  }

  arr[j + 1] = key;
}

function mergeSort(unsortedArray) {
  // No need to sort the array if the array only has one element or empty
  if (unsortedArray.length <= 1) {
    return unsortedArray;
  }
  // In order to divide the array in half, we need to figure out the middle
  const middle = Math.floor(unsortedArray.length / 2);

  // This is where we will be dividing the array into left and right
  const left = unsortedArray.slice(0, middle);
  const right = unsortedArray.slice(middle);

  // Using recursion to combine the left and right
  return merge(
    mergeSort(left), mergeSort(right)
  );
}

export { quickSort, bubbleSort, insertionSort, mergeSort };

//var sortedArray = quickSort(items, 0, items.length - 1);