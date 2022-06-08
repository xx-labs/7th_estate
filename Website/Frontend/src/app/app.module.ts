import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { AppComponent } from './app.component';
import { VoteFormComponent } from './VoteForm/VoteForm.component';
import { FormsModule } from "@angular/forms";
import { HttpClientModule } from "@angular/common/http";
import { WaitingReceiptComponent } from './waiting-receipt/waiting-receipt.component';


@NgModule({
  declarations: [
    AppComponent,
      VoteFormComponent,
      WaitingReceiptComponent,
   ],
  imports: [
    BrowserModule,
    FormsModule,
    HttpClientModule,
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
