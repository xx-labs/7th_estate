import { Component, OnInit, Input } from '@angular/core';
import { VoteService } from '../VoteService.service';

@Component({
  selector: 'app-waiting-receipt',
  templateUrl: './waiting-receipt.component.html',
  styleUrls: ['./waiting-receipt.component.css']
})
export class WaitingReceiptComponent implements OnInit {
  @Input() hash: string;
  @Input() status: string;
  receipt: string;
  id: any;
  interval: number = 10000;

  constructor(private voteservice: VoteService) { }

  ngOnInit() {
    this.checkReceipt();
    this.id = setInterval(() => this.timeout(), this.interval)
  }

  checkReceipt() {
    console.log("Checking receipt")
    this.voteservice.checkReceipt(this.hash)
      .subscribe(response => {
        console.log(response);
        this.receipt = response['receipt'] ? JSON.parse(response['receipt']) : null;
        this.status = response ['status'] || this.status;
      })
  }

  timeout() {
    if (this.receipt){
      clearInterval(this.id);
      return;
    }
    this.checkReceipt();
  }

  ngOnDestroy() {
    clearInterval(this.id);
  }
}
