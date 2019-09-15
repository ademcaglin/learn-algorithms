import { assert } from "chai";
import { find_diff, remove } from "../src/find_anagrams";


describe("Diff test", function () {
    it("should find diff", () => {
        assert.equal(
            find_diff("aabaacd", "aacb"), "aad"
        );
    });
});

describe("Remove test", function () {
    it("should remove", () => {
        let arr = [1,2,3];
        remove(arr, 2);
        assert.deepEqual(
            arr, [1,3]
        );
    });
});