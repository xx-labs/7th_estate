import { Component, OnInit } from '@angular/core';
import { VoteService } from '../VoteService.service';
import { ValidateVotecodeService } from '../ValidateVotecode.service';

@Component({
  selector: 'app-VoteForm',
  templateUrl: './VoteForm.component.html',
  styleUrls: ['./VoteForm.component.css']
})
export class VoteFormComponent implements OnInit {

  votecode: string;

  error: string;
  success: string;
  hash: string;
  receipt: string;
  status: string

  csrf: string;

  constructor(private votservice: VoteService,
              public validatecode: ValidateVotecodeService) { }

  ngOnInit() {
    this.getCsrf();
  }

  postVote() {
    // Clear error messages
    this.error = this.success = "";

    // Check parity of votecode
    if (this.validatecode.checkVotecode(this.votecode)){
      // Submit vote to backend
      this.votservice.postVote(this.votecode, this.csrf)
        .subscribe(response => {
          console.log(response);
          this.error = response["errormessage"] || null;
          this.receipt = response["receipt"] || null;
          this.status = response['status'] || null;
          this.hash = response['hash'] || null;
        });
    }
    else
      this.error = "The vote code you entered is wrong"
  }

  getCsrf() {
    this.votservice.getCsrf()
      .subscribe(response => {
        console.log(response);
        this.csrf = response['csrf'];
    })
  }

  keyup(event: any) {
    const pattern = /[0-9\-]/;
    if (!pattern.test(event.key)) {
      // invalid character, prevent input
      event.preventDefault();
    }
  }
}

