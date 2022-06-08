import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { WaitingReceiptComponent } from './waiting-receipt.component';

describe('WaitingReceiptComponent', () => {
  let component: WaitingReceiptComponent;
  let fixture: ComponentFixture<WaitingReceiptComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ WaitingReceiptComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(WaitingReceiptComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
