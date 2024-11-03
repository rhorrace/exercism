#include "high_scores.h"

int32_t latest(const int32_t *scores, size_t scores_len) {
    if(!scores) {
        return -1;
    }

    return scores[scores_len - 1];
}

int32_t personal_best(const int32_t *scores, size_t scores_len) {
    if(!scores) {
        return -1;
    }

    int max_score = 0;

    for(size_t i = 0;i < scores_len;++i) {
        max_score = MAX(max_score, scores[i]);
    }

    return max_score;
}

size_t personal_top_three(const int32_t *scores, size_t scores_len, int32_t *output) {
    size_t output_len = 3;

    if(scores_len < 3) output_len = scores_len;

    for(size_t i =0; i < scores_len;++i) {
        if(scores[i] <= output[2]) {
            continue;
        }

        if(scores[i] <= output[1]) {
            output[2] = scores[i];
            continue;
        }

        if(scores[i] <= output[0]) {
            output[2] = output[1];
            output[1] = scores[i];
            continue;
        }
        
        output[2] = output[1];
        output[1] = output[0];
        output[0] = scores[i];
    }

    return output_len;
}
