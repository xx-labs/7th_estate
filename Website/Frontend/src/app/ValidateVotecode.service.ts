import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class ValidateVotecodeService {

  VOTE_CODE_GROUP_SIZE: number = 4;
  VOTE_CODE_GROUP_SIZE_P: number = this.VOTE_CODE_GROUP_SIZE + 1;
  VOTE_CODE_NUM_GROUPS: number = 4;
  VOTE_CODE_LENGTH: number = this.VOTE_CODE_NUM_GROUPS * this.VOTE_CODE_GROUP_SIZE_P;
  TOTAL_VOTE_LENGTH = this.VOTE_CODE_LENGTH + this.VOTE_CODE_NUM_GROUPS - 1;

  constructor() { }

  checkVotecode(votecode: string) {
    let codegroups = votecode.split("-");
    for (let i = 0; i < this.VOTE_CODE_NUM_GROUPS; i++){
      if (!this.checkParity(codegroups[i]))
        return false;
    }
    return true
  }

  checkParity(code: string){
    if (code.length != this.VOTE_CODE_GROUP_SIZE_P)
      return false;

    let parity = +code.slice(-1);
    let code_sum = code.slice(0, -1).split('').map(c => +c).reduce((sum, cur) => {return sum + cur});
    let code_parity = (10 * this.VOTE_CODE_NUM_GROUPS - code_sum) % 10;
    return parity == code_parity;
  }
}
